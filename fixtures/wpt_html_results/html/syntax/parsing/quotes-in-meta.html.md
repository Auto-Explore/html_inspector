# html/syntax/parsing/quotes-in-meta.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/quotes-in-meta.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta http-equiv="Content-Type" content='charset="windows-1251'>
<meta charset=windows-1250>
<title></title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<link rel=help href="https://html.spec.whatwg.org/#algorithm-for-extracting-a-character-encoding-from-a-meta-element">
<script>
test(function() {
    assert_equals(document.characterSet, "windows-1250");
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.charset_and_content_type.disallowed",
      "message": "A document must not include both a “meta” element with an “http-equiv” attribute whose value is “content-type”, and a “meta” element with a “charset” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 108,
        "byte_start": 81,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “windows-1250” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 108,
        "byte_start": 81,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 116,
        "byte_start": 109,
        "col": 1,
        "line": 4
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
  "source_name": "html/syntax/parsing/quotes-in-meta.html"
}
```
