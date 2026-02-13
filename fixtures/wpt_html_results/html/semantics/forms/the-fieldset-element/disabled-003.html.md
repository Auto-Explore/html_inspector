# html/semantics/forms/the-fieldset-element/disabled-003.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-fieldset-element/disabled-003.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Disable nested fieldsets with focused element</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-fieldset-element">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1427047">
<link rel="author" href="mailto:xiaochengh@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=container1>
  <fieldset id=target1>
    <legend>foo</legend>
    <fieldset>
      <legend>bar</legend>
      <input id=input1>
    </fieldset>
  </fieldset>
</div>
<script>
test(() => {
  input1.focus();
  target1.disabled = true;
  assert_not_equals(document.activeElement, input1);
}, 'Disable light-nested fieldsets should not crash');
</script>

<div id=container2></div>
<script>
test(() => {
  let n = 100;
  let markup = '<fieldset><legend>foo</legend>'.repeat(n) +
               '<input id=input2>' + '</fieldset>'.repeat(n);
  container2.innerHTML = markup;
  input2.focus();
  container2.firstChild.disabled = true;
  assert_not_equals(document.activeElement, input2);
}, 'Disable deep-nested fieldsets should not hang');
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
  "source_name": "html/semantics/forms/the-fieldset-element/disabled-003.html"
}
```
