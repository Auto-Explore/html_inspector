# html/semantics/document-metadata/the-meta-element/http-equiv-and-name-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/http-equiv-and-name-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Setting both http-equiv and name attributes on a meta element</title>
<meta http-equiv=content-language name=color-scheme content=de-DE>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  // We don't attempt to test the color-scheme here because "de-DE" is not a valid
  // value for it.

  test(() => {
    assert_equals(document.querySelector(":root:lang(de-DE)"), document.documentElement);
  }, "<meta> set the content-language to de-DE");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.http_equiv.content_language.obsolete",
      "message": "Using the “meta” element to specify the document-wide default language is obsolete. Consider specifying the language on the root element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 159,
        "byte_start": 93,
        "col": 1,
        "line": 3
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
  "source_name": "html/semantics/document-metadata/the-meta-element/http-equiv-and-name-2.html"
}
```
