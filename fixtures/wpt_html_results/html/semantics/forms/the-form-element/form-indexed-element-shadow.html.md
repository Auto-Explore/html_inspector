# html/semantics/forms/the-form-element/form-indexed-element-shadow.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-indexed-element-shadow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>form.elements: indexed access reflects DOM order, not flat tree</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<form id="target">
  <div id="host">
    <template shadowrootmode="open">
      <slot name="first"></slot>
      <slot name="second"></slot>
    </template>
    <input id="first" slot="second">
    <input id="second" slot="first">
  </div>
</form>
<script>
test(function() {
  let target = document.getElementById("target");
  let host = document.getElementById("host");
  assert_true(!!host.shadowRoot, "Should have a shadow tree");
  assert_equals(target.elements[0], first, "form.elements reflects DOM order, not flat tree order");
  assert_equals(target.elements[1], second);
});
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
  "source_name": "html/semantics/forms/the-form-element/form-indexed-element-shadow.html"
}
```
