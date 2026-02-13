# html/semantics/interestfor/interestfor-keyboard-behavior.tentative.html

Counts:
- errors: 1
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-keyboard-behavior.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<meta name="timeout" content="long">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>
<script src="/html/semantics/popovers/resources/popover-utils.js"></script>

<button data-testcase="<button>" interestfor=target>Button</button>

<a data-testcase="<a>" href=foo interestfor=target>Link</a>

<img src="/images/blue.png" usemap="#map">
<map id=map>
  <area data-testcase="<area>" interestfor=target href="/" shape=default>
</map>

<svg viewBox="0 0 100 100" style="width: 100px" xmlns="http://www.w3.org/2000/svg">
  <a data-testcase="SVG <a>" href=foo interestfor=target>
    <text x=50 y=90>SVG A</text>
  </a>
</svg>

<menulist style="display:block; inset:auto; top: 300px">
  <menuitem data-testcase="<menuitem>" interestfor=target>menuitem</menuitem>
</menulist>

<div id=target popover>Popover</div>
<button id="otherbutton">Other button</button>
<button id="another" interestfor=anothertarget>Another Button</button>
<div id=anothertarget popover>Another Popover</div>

<style>
  [interestfor] {
    interest-delay: 0s;
  }
  [interestfor].longhide {
    interest-delay-end: 10000s;
  }
</style>

<script>
const menuSupported = document.createElement('menuitem') instanceof HTMLMenuItemElement;
const menu = document.querySelector('menulist:has(menuitem[data-testcase])');
if (menuSupported) {
  // A menulist needs to be open to be interactive.
  menu.showPopover();
}
let allInterestForElements = [...document.querySelectorAll('[data-testcase]')];
if (!menuSupported) {
  allInterestForElements = allInterestForElements.filter(el => el.localName !== 'menuitem');
}
assert_true(allInterestForElements.length > 0);

function verifyInterest(onlyElements,description) {
  if (!(onlyElements instanceof Array)) {
    onlyElements = [onlyElements];
  }
  [...allInterestForElements, another].forEach(el => {
    const expectInterest = onlyElements.includes(el);
    assert_equals(el.matches(':interest-source'),expectInterest,`${description}, element ${el.dataset.testcase} should ${expectInterest ? "" : "NOT "}have interest`);
  })
}
allInterestForElements.forEach(el => {
  const description = el.dataset.testcase;
  promise_test(async function (t) {
    t.add_cleanup(() => otherbutton.focus());
    target.hidePopover(); // Just in case
    await focusOn(el);
    assert_equals(document.activeElement,el,'Elements should all be focusable');
    assert_true(target.matches(':popover-open'),'Focusing should trigger interest');
    verifyInterest(el,`After show interest in ${description}`);
    await focusOn(otherbutton);
    assert_not_equals(document.activeElement,el);
    assert_false(target.matches(':popover-open'),'Blurring should trigger lose interest');
    verifyInterest(undefined,`After lose interest in ${description}`);
  },`Basic keyboard focus behavior, ${description}`);

  promise_test(async function (t) {
    t.add_cleanup(() => otherbutton.focus());
    target.hidePopover(); // Just in case
    await focusOn(el);
    assert_true(target.matches(':popover-open'),'Focusing should trigger interest');
    verifyInterest(el,`After show interest in ${description}`);
    await sendLoseInterestHotkey();
    assert_false(target.matches(':popover-open'),'Pressing lose interest hot key should trigger lose interest');
    verifyInterest(undefined,`After lose interest in ${description}`);
    await focusOn(otherbutton);
    assert_not_equals(document.activeElement,el);
    assert_false(target.matches(':popover-open'),'Blurring should do nothing at this point');
    verifyInterest(undefined,`After blurring ${description}`);
  },`Lose interest hot key behavior, ${description}`);

  promise_test(async function (t) {
    t.add_cleanup(() => otherbutton.focus());
    // Ensure blurring doesn't immediately lose interest:
    el.classList.add('longhide');
    t.add_cleanup(() => (el.classList.remove('longhide')));
    target.hidePopover(); // Just in case
    await focusOn(el);
    assert_true(target.matches(':popover-open'),'Focusing should trigger interest');
    verifyInterest(el,`After show interest in ${description}`);
    await focusOn(otherbutton);
    assert_not_equals(document.activeElement,el);
    assert_true(target.matches(':popover-open'),'Blurring should not immediately lose interest');
    verifyInterest(el,`After blurring ${description}`);
    // Send lose interest hot key to the other button (not the invoker):
    await sendLoseInterestHotkey();
    assert_false(target.matches(':popover-open'),'Pressing lose interest hot key should trigger lose interest');
    verifyInterest(undefined,`After lose interest in ${description}`);
  },`Lose interest hot key behavior with element not focused, ${description}`);

  promise_test(async function (t) {
    t.add_cleanup(() => otherbutton.focus());
    target.hidePopover(); // Just in case
    target.addEventListener('interest', (e) => e.preventDefault(), {once: true});
    await focusOn(el);
    assert_false(target.matches(':popover-open'));
    verifyInterest(undefined,`Nothing has interest, ${description}`);
  }, `canceling the interest event stops behavior, ${description}`);

  let events = [];
  function addListeners(t,element) {
    const signal = t.get_signal();
    element.addEventListener('interest',(e) => events.push(`${e.target.id} interest`),{signal});
    element.addEventListener('loseinterest',(e) => events.push(`${e.target.id} loseinterest (${e.cancelable ? 'cancelable' : 'not cancelable'})`),{signal});
  }
  promise_test(async function (t) {
    t.add_cleanup(() => otherbutton.focus());
    target.hidePopover(); // Just in case
    anothertarget.hidePopover(); // Just in case
    events = [];
    addListeners(t,target);
    addListeners(t,anothertarget);
    await focusOn(el);
    assert_array_equals(events,['target interest'],'first hotkey');
    verifyInterest(el,`After show interest in ${description}`);
    await focusOn(another);
    assert_array_equals(events,['target interest','target loseinterest (cancelable)','anothertarget interest'],
        'showing interest in another trigger should lose interest in the first, then gain interest in second');
    verifyInterest(another,`After show interest in ${another.id}`);
    await sendLoseInterestHotkey();
    assert_array_equals(events,['target interest','target loseinterest (cancelable)','anothertarget interest','anothertarget loseinterest (not cancelable)']);
    verifyInterest(undefined,`After lose interest in ${another.id}`);
    assert_false(target.matches(':popover-open'));
    assert_false(anothertarget.matches(':popover-open'));
  }, `Showing interest in a second element loses interest in the first, ${description}`);

  promise_test(async function (t) {
    t.add_cleanup(() => otherbutton.focus());
    target.hidePopover(); // Just in case
    anothertarget.hidePopover(); // Just in case
    events = [];
    addListeners(t,target);
    addListeners(t,anothertarget);
    await focusOn(el);
    assert_array_equals(events,['target interest'],'setup');
    verifyInterest(el,`After show interest in ${description}`);
    const signal = t.get_signal();
    let shouldCancelLoseInterest = true;
    target.addEventListener('loseinterest',(e) => {
      if (shouldCancelLoseInterest) {
        e.preventDefault();
      }
    },{signal});
    await focusOn(another);
    assert_array_equals(events,['target interest','target loseinterest (cancelable)','anothertarget interest','target loseinterest (cancelable)'],
        'the loseinterest listener should fire but get cancelled, anothertarget should still get interest, and that should close the first target popover firing another loseinterest');
    events = [];
    verifyInterest([el,another],`${description} should still have interest because loseinterest was cancelled`);
    assert_false(target.matches(':popover-open'),'anothertarget popover opens, closing target');
    assert_true(anothertarget.matches(':popover-open'));
    await sendLoseInterestHotkey();
    assert_array_equals(events,['anothertarget loseinterest (not cancelable)', 'target loseinterest (not cancelable)'],'Lose interest hot key loses interest in all elements');
    assert_false(target.matches(':popover-open'));
    assert_false(anothertarget.matches(':popover-open'));
    verifyInterest(undefined,`Nothing has interest now`);
  }, `Canceling loseinterest caused by keyboard-gained interest cancels interest, ${description}`);
});
</script>

<button id="esc_invoker1" class="longhide" interestfor="esc_target1">ESC Invoker 1</button>
<div id="esc_target1">Non-popover target for ESC test</div>
<button id="esc_invoker2" class="longhide" interestfor="esc_target2">ESC Invoker 2</button>
<div id="esc_target2">Non-popover target for ESC test</div>
<button id="esc_invoker3" class="longhide" interestfor="esc_target3">ESC Invoker 3</button>
<div id="esc_target3">Non-popover target for ESC test</div>

<script>
promise_test(async function (t) {
  const invoker1 = document.getElementById('esc_invoker1');
  const target1 = document.getElementById('esc_target1');
  const invoker2 = document.getElementById('esc_invoker2');
  const target2 = document.getElementById('esc_target2');
  const invoker3 = document.getElementById('esc_invoker3');
  const target3 = document.getElementById('esc_target3');
  const otherbutton = document.getElementById('otherbutton');
  t.add_cleanup(() => otherbutton.focus());

  let events = [];
  const signal = t.get_signal();
  [target1, target2, target3].forEach(target => {
    target.addEventListener('interest',(e) => events.push(`${e.source.id} interest`),{signal});
    target.addEventListener('loseinterest',(e) => events.push(`${e.source.id} loseinterest`),{signal});
    // These loseinterest events should not be cancelable:
    target.addEventListener('loseinterest',(e) => e.preventDefault(),{signal});
  });

  // Invoke them in non-tree order:
  await focusOn(invoker1);
  await focusOn(invoker3);
  await focusOn(invoker2);
  assert_array_equals(events,
    ['esc_invoker1 interest','esc_invoker3 interest','esc_invoker2 interest'],
    'Events after gaining interest');
  events = [];

  // Hit ESC once, while focused on the body
  document.body.focus();
  await waitForRender();
  assert_true(invoker1.matches(':interest-source'), 'invoker1 should still have interest');
  assert_true(invoker3.matches(':interest-source'), 'invoker3 should still have interest');
  assert_true(invoker2.matches(':interest-source'), 'invoker2 should still have interest');
  const kEscape = '\uE00C';
  await new test_driver.Actions()
    .keyDown(kEscape)
    .keyUp(kEscape)
    .send();
  await waitForRender();
  assert_false(invoker2.matches(':interest-source'), 'invoker2 should lose interest');
  assert_false(invoker1.matches(':interest-source'), 'invoker1 should lose interest');
  assert_false(invoker3.matches(':interest-source'), 'invoker3 should lose interest');
  assert_array_equals(events, [
    'esc_invoker2 loseinterest', 'esc_invoker3 loseinterest', 'esc_invoker1 loseinterest'],
    'ESC should lose interest in all invokers, in reverse order');
}, 'ESC key dismisses all interest invokers');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 787,
        "byte_start": 745,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1129,
        "byte_start": 1073,
        "col": 1,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1129,
        "byte_start": 1073,
        "col": 1,
        "line": 29
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1188,
        "byte_start": 1132,
        "col": 3,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1188,
        "byte_start": 1132,
        "col": 3,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1436,
        "byte_start": 1429,
        "col": 1,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “map”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 787,
        "byte_start": 745,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interestfor/interestfor-keyboard-behavior.tentative.html"
}
```
