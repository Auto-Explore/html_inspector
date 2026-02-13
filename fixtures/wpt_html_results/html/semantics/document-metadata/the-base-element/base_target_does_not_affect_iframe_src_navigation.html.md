# html/semantics/document-metadata/the-base-element/base_target_does_not_affect_iframe_src_navigation.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-base-element/base_target_does_not_affect_iframe_src_navigation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<base id="base" target="_blank">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe id="i" src="about:blank"></iframe>
<script>
async_test(function(t) {
  window.onmessage = () => t.done();
  i.src = "data:text/html,This should navigate the iframe<script>top.postMessage('done', '*');</sc" + "ript>";
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
        "byte_end": 32,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/semantics/document-metadata/the-base-element/base_target_does_not_affect_iframe_src_navigation.html"
}
```
