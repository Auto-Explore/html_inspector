# html/rendering/non-replaced-elements/flow-content-0/form-margin-quirk.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/flow-content-0/form-margin-quirk.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!-- quirks -->
<title>form margin quirk</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<style>
form { writing-mode: vertical-lr; }
#ref { margin: 0 1em 0 0; }
</style>
<form id=form></form>
<div id=ref></div>
<script>
test(() => {
  const formStyle = getComputedStyle(document.getElementById('form'));
  const refStyle = getComputedStyle(document.getElementById('ref'));
  assert_equals(formStyle.marginTop, refStyle.marginTop, 'marginTop');
  assert_equals(formStyle.marginRight, refStyle.marginRight, 'marginRight');
  assert_equals(formStyle.marginBottom, refStyle.marginBottom, 'marginBottom');
  assert_equals(formStyle.marginLeft, refStyle.marginLeft, 'marginLeft');
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 23,
        "byte_start": 16,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/flow-content-0/form-margin-quirk.html"
}
```
