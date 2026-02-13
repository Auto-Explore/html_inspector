# html/semantics/interestfor/interestfor-basic-delays.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-basic-delays.tentative.html",
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

<body>
<script>
// NOTE. This is just a single, basic test of the delays for `interestfor`.
// The `interest-delay-end` and `interest-delay-start` tests take a
// much more in-depth look at *how* the features work, but due to some nuances
// (please read the "NOTE" sections in those tests), they could end up passing
// on browsers that don't actually support this feature. So this test takes
// considerably longer, but should only pass on browsers that implement the
// `interestfor` attribute.

const invokerDelays = 2000; // The CSS delay setting. Should be long enough to ensure it works, even on slow bots.
const hoverWaitTime = 4000; // How long to wait to cover the delay for sure.

promise_test(async (t) => {
  assert_true(hoverWaitTime >= 2*invokerDelays,'invokerDelays is the value from CSS, hoverWaitTime should be longer than that');
  let {popover:div, invoker, unrelated} = await createPopoverAndInvokerForHoverTests(t, invokerDelays, invokerDelays);
  div.removeAttribute('popover'); // No longer a popover
  let events = [];
  div.addEventListener('interest',() => events.push('interest'));
  div.addEventListener('loseinterest',() => events.push('loseinterest'));
  // Quickly do the entire test.
  await hoverOver(invoker);
  const checkpoint1 = [...events];
  await waitForHoverTime(hoverWaitTime);
  const checkpoint2 = [...events];
  await hoverOver(unrelated);
  const checkpoint3 = [...events];
  await waitForHoverTime(hoverWaitTime);
  const checkpoint4 = [...events];
  // Now test for expectations.
  assert_array_equals(checkpoint1,[],'After initial hover, no events fired yet');
  assert_array_equals(checkpoint2,['interest'],'After wait time, interest gets fired');
  assert_array_equals(checkpoint3,['interest'],'After initial de-hover, no events fired yet');
  assert_array_equals(checkpoint4,['interest','loseinterest'],'After wait time, loseinterest gets fired');
},'Basic hover interest and loseinterest behavior');
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/semantics/interestfor/interestfor-basic-delays.tentative.html"
}
```
