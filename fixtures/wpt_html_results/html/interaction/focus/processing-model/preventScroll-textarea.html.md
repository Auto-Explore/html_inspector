# html/interaction/focus/processing-model/preventScroll-textarea.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/processing-model/preventScroll-textarea.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>focus(options) - preventScroll on textarea element</title>
<link rel="author" href="https://mozilla.org" title="Mozilla">
<link rel="author" href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1634153">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<div style="height: 200vh"></div>
<textarea>ABCD</textarea>
<input value="EFGH">
<button></button>
<div style="height: 200vh"></div>
<script>
promise_test(async function(t) {
  await new Promise(resolve => window.addEventListener("load", resolve));
  let elements = document.querySelectorAll("textarea, input, button");
  assert_equals(elements.length, 3, `Precondition`);
  for (let element of elements) {
    let name = element.nodeName;
    assert_equals(window.scrollY, 0, `${name}: Precondition`);
    element.focus({ preventScroll: true });
    assert_equals(window.scrollY, 0, `${name}: Should not have scrolled`);
    assert_equals(document.activeElement, element, `${name}: Should have been focused`);

    // Wait a couple event loop turns because the original bug was triggered off
    // an async event.
    await new Promise(resolve => t.step_timeout(resolve, 0));
    await new Promise(resolve => t.step_timeout(resolve, 0));
    assert_equals(window.scrollY, 0, `${name}: Should not have scrolled after a couple event loop ticks`);
    assert_equals(document.activeElement, element, `${name}: Should remain focused`);

    // Also wait for rendering, just out of paranoia.
    await new Promise(resolve => requestAnimationFrame(resolve));
    await new Promise(resolve => requestAnimationFrame(resolve));

    assert_equals(window.scrollY, 0, `${name}: Should not have scrolled after rendering`);
    assert_equals(document.activeElement, element, `${name}: Should remain focused after rendering`);
  }
}, "preventScroll: true on a textarea element");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/interaction/focus/processing-model/preventScroll-textarea.html"
}
```
