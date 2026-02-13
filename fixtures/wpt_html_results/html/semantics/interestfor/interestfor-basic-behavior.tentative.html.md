# html/semantics/interestfor/interestfor-basic-behavior.tentative.html

Counts:
- errors: 1
- warnings: 9
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-basic-behavior.tentative.html",
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
<link rel="help" href="https://github.com/whatwg/html/pull/11006" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<meta name=variant content=?method=hover>
<meta name=variant content=?method=focus>

<button data-testcase="<button>" interestfor=target>Button</button>

<a data-testcase="<a>" href="#" interestfor=target>Link</a>

<img src="/images/blue.png" usemap="#map" id=areatarget>
<map id=map>
  <area data-testcase="<area>" data-hover="areatarget" interestfor=target href="/" shape=default>
</map>

<svg viewBox="0 0 100 100" style="width: 100px" xmlns="http://www.w3.org/2000/svg">
  <a data-testcase="SVG <a>" href="#" interestfor=target>
    <text x=50 y=90>SVG A</text>
  </a>
</svg>

<a data-testcase="Broken img" href="#" interestfor=target>
  <img src="broken" width="50" height="50">
</a>

<a data-testcase="SVG <use>" href="#" interestfor=target>
  <svg width="50" height="50" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
    <use xlink:href="#thick-star"></use>
  </svg>
</a>
<svg style="display: none;">
  <symbol id="thick-star" viewBox="0 0 24 24">
    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"
          fill="yellow" stroke="black" stroke-width="2" stroke-linejoin="round"/>
  </symbol>
</svg>

<menulist style="display:block; inset:auto; top: 300px">
  <menuitem data-testcase="<menuitem>" interestfor=target>menuitem</menuitem>
</menulist>

<div id=target>Target</div>
<button id=otherbutton>Other button</button>

<style>
  [interestfor] {
    interest-delay: 0s;
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
function reinsert(element) {
  const parent = element.parentElement;
  const nextSibling = element.nextSibling;
  element.remove();
  parent.insertBefore(element,nextSibling);
}
function preventEvent(shouldCancel,el,type) {
  if (shouldCancel) {
    assert_not_equals(type,'focusin','focusin can\'t be cancelled');
    assert_not_equals(type,'focusout','focusout can\'t be cancelled');
    el.addEventListener(type, (e) => e.preventDefault(), {once:true});
  }
}

const urlParams = new URLSearchParams(window.location.search);
method = urlParams.get('method');
['none','cancel-trigger','cancel-lose'].forEach(cancelEvent => {
  allInterestForElements.forEach(el => {
    const description = `${el.dataset.testcase}, ${cancelEvent}, ${method}`;
    promise_test(async function (t) {
      t.add_cleanup(() => {
        reinsert(el);
        reinsert(target);
      });
      assert_false(el.matches(':interest-source'),'setup');
      assert_false(target.matches(':interest-target'),'setup');
      const signal = t.get_signal();
      let interestCount = 0;
      let loseInterestCount = 0;
      target.addEventListener('interest', (e) => (++interestCount), {signal});
      target.addEventListener('loseinterest', () => (++loseInterestCount), {signal});
      const cancelTrigger = cancelEvent === 'cancel-trigger';
      const cancelLose = cancelEvent === 'cancel-lose';
      assert_true(cancelTrigger || cancelLose || cancelEvent === 'none');

      switch (method) {
        case 'hover':
          preventEvent(cancelTrigger,el,'mouseover');
          hovertarget = el;
          if (el.dataset.hover) {
            hovertarget = document.getElementById(el.dataset.hover);
          }
          await hoverOver(hovertarget)
          break;
        case 'focus':
          if (cancelTrigger) {
            return; // focusin cannot be cancelled, nothing to test
          }
          await focusOn(el);
          break;
        default:
          assert_unreached();
      }
      assert_equals(loseInterestCount, 0, 'Lose interest should not be fired yet');
      assert_equals(interestCount, 1, 'Interest should be fired (cancelling the trigger event shouldn\'t cancel interest)');
      assert_true(el.matches(':interest-source'),':interest-source should match');
      interestCount = 0;

      switch (method) {
        case 'hover':
          preventEvent(cancelLose,el,'mouseout');
          await hoverOver(otherbutton);
          break;
        case 'focus':
          if (cancelLose) {
            return; // focusout cannot be cancelled, nothing to test
          }
          await focusOn(otherbutton);
          break;
        default:
          assert_unreached();
      }
      assert_equals(interestCount, 0, 'No new interest event should be fired');
      assert_equals(loseInterestCount, 1, 'Lose interest event should be fired (cancelling the trigger event shouldn\'t cancel loseinterest)' );
      assert_false(el.matches(':interest-source'),':interest-source should not match');
    },`Basic behavior, ${description}`);
  });
});
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
        "byte_end": 879,
        "byte_start": 823,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 1291,
        "byte_start": 1250,
        "col": 3,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1936,
        "byte_start": 1880,
        "col": 1,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1936,
        "byte_start": 1880,
        "col": 1,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1995,
        "byte_start": 1939,
        "col": 3,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1995,
        "byte_start": 1939,
        "col": 3,
        "line": 49
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 2109,
        "byte_start": 2102,
        "col": 1,
        "line": 55
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “map”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 879,
        "byte_start": 823,
        "col": 1,
        "line": 21
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
  "source_name": "html/semantics/interestfor/interestfor-basic-behavior.tentative.html"
}
```
