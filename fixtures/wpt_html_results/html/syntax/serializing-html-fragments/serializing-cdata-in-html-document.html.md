# html/syntax/serializing-html-fragments/serializing-cdata-in-html-document.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/serializing-html-fragments/serializing-cdata-in-html-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Serializing CDATA in an HTML document</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/parsing.html#serialising-html-fragments">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(t => {
  const doc = new DOMParser().parseFromString('<svg xmlns="http://www.w3.org/2000/svg"><![CDATA[<img>]]></svg>', 'application/xml');
  const el = document.adoptNode(doc.documentElement);
  assert_equals(el.outerHTML, '<svg xmlns="http://www.w3.org/2000/svg">&lt;img&gt;</svg>');
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
  "source_name": "html/syntax/serializing-html-fragments/serializing-cdata-in-html-document.html"
}
```
