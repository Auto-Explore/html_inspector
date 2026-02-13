# html/semantics/forms/the-input-element/range-restore-oninput-onchange-event.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/range-restore-oninput-onchange-event.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/7283">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1266468">

<link rel=author href="mailto:gulukesh@gmail.com">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1131234">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
function runTest(type, testValue) {
  promise_test(async () => {
    const w = window.open(`resources/${type}-restore-events.html`);
    // Unfortunately, navigating |w| doesn't fire load events in this parent
    // window, so we have to make the child window manually tell this parent
    // window when it has loaded.
    await new Promise(resolve => window.loadResolver = resolve);
    // We can't navigate the child window until after a setTimeout.
    await new Promise(resolve => step_timeout(resolve, 0));

    assert_not_equals(
      w.document.querySelector('input').value,
      testValue,
      `Test shouldn't start with the new value already in the input.`);
    w.document.querySelector('input').value = testValue;

    w.location.href = 'resources/loadresolver.html';
    await new Promise(resolve => window.loadResolver = resolve);

    w.history.back();
    await new Promise(resolve => window.loadResolver = resolve);
    // The value doesn't get restored until after a setTimeout.
    await new Promise(resolve => step_timeout(resolve, 0));

    assert_equals(w.document.querySelector('input').value, testValue,
      'The input should have its value restored.');

    assert_false(w.seeninput || false,
      'The input event should not have been fired after restoration.');
    assert_false(w.seenchange || false,
      'The change event should not have been fired after restoration.');

    w.close();
  }, `Verifies that form restoration does not fire input or change events for <input type=${type}>.`);
}

runTest('range', '8');
runTest('text', 'foo');
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
  "source_name": "html/semantics/forms/the-input-element/range-restore-oninput-onchange-event.https.html"
}
```
