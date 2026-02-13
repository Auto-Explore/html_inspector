# html/dom/elements/global-attributes/mapped-attribute-adopt-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/mapped-attribute-adopt-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Adoption doesn't mess with mapped attributes</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1636516">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div hidden="unlikely">Should be hidden</div>
<script>
test(function() {
  var tmpl = document.createElement("template");
  var fragment = tmpl.content;
  var newEl = document.createElement("div");
  newEl.setAttribute("hidden", "unlikely");
  fragment.append(newEl);
  document.adoptNode(newEl);
  assert_equals(
    getComputedStyle(document.querySelector("div")).display,
    "none",
    "hidden attribute should have an effect"
  );
}, "Adoption of an unrelated node shouldn't prevent mapped attributes from applying");
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
  "source_name": "html/dom/elements/global-attributes/mapped-attribute-adopt-001.html"
}
```
