# html/semantics/interestfor/interestfor-event-timing.tentative.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-event-timing.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<button interestfor=popover id=invoker>Button</button>
<div popover id=popover>Target</div>
<button id=unrelated>Unrelated</div>
<style>
  [interestfor] {
    interest-delay: 0s;
  }
</style>

<script>
promise_test(async (t) => {
  const invoker = document.getElementById('invoker');
  const popover = document.getElementById('popover');
  const unrelated = document.getElementById('unrelated');

  let interestFired = 0;
  let loseInterestFired = 0;
  let listenerErrors = [];
  function assert_in_listener(cond, expectation, msg) {
    if (cond != expectation) {
      listenerErrors.push(msg);
    }
  }
  popover.addEventListener('interest', (e) => {
    ++interestFired;
    assert_in_listener(invoker.matches(':interest-source'),false,':interest-source should not yet match in the event handler');
    assert_in_listener(popover.matches(':interest-target'),false,':interest-target should not yet match in the event handler');
    assert_in_listener(popover.matches(':popover-open'),false,'popover shouldn\'t be open yet');
    if (interestFired === 1) {
      e.preventDefault();
    }
  });
  popover.addEventListener('loseinterest', (e) => {
    ++loseInterestFired;
    assert_in_listener(invoker.matches(':interest-source'),true,':interest-source should match in the event handler');
    assert_in_listener(popover.matches(':interest-target'),true,':interest-target should match in the event handler');
    assert_in_listener(popover.matches(':popover-open'),true,'popover should still be open');
    if (loseInterestFired === 1) {
      e.preventDefault();
    }
  });

  // Hover once, and cancel the event:
  await hoverOver(invoker);
  assert_equals(interestFired, 1, 'The `interest` event should have fired once (and event canceled)');
  await hoverOver(unrelated);
  assert_equals(interestFired, 1, 'No further interest events');
  assert_equals(loseInterestFired, 0, 'No loseinterest events yet (the `interest` event was canceled, so we can\'t "lose" interest we never gained)');

  // Hover again, and don't cancel the event:
  await hoverOver(invoker);
  assert_equals(interestFired, 2, 'The `interest` event should have fired twice (event not canceled the second time)');
  assert_equals(loseInterestFired, 0, 'Still no loseinterest events');

  // De-hover, and cancel the event:
  await hoverOver(unrelated);
  assert_equals(interestFired, 2, 'No further interest events');
  assert_equals(loseInterestFired, 1, 'The `loseinterest` event should have fired once (and event canceled)');

  // De-hover again, and don't cancel the event:
  await hoverOver(invoker); // Have to re-hover to get another interest event
  assert_equals(interestFired, 2, 'No additional `interest` event because we cancelled loseinterest, so we still had interest');
  await hoverOver(unrelated);
  assert_equals(interestFired, 2, 'No further interest events');
  assert_equals(loseInterestFired, 2, 'The `loseinterest` event should have fired once (event not canceled the second time)');

  assert_false(popover.matches(':popover-open'),'popover should be closed at the end');

  // Make sure none of the conditions within the event listeners failed.
  assert_equals(listenerErrors.length,0,listenerErrors.join(', '));
},'Event and pseudo-class timing for interest and loseinterest');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “div”.",
      "severity": "Error",
      "span": {
        "byte_end": 664,
        "byte_start": 658,
        "col": 31,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 672,
        "byte_start": 665,
        "col": 1,
        "line": 16
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
  "source_name": "html/semantics/interestfor/interestfor-event-timing.tentative.html"
}
```
