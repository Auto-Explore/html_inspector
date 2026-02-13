# html/syntax/parsing/meta-inhead-insertion-mode.html

Counts:
- errors: 1
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/meta-inhead-insertion-mode.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>Encoding specified in the "charset" attribute should have precedence over "content" attribute.</title>
<meta http-equiv="Content-Type" content="text/html; charset=koi8-r" charset="iso-8859-15">
<link rel="help" href="https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inhead">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
test(function () {
  assert_equals(document.characterSet, "ISO-8859-15");
}, "Encoding specified in the 'charset' attribute should have precedence over 'content' attribute.");
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.charset.content.disallowed",
      "message": "Attribute “content” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 230,
        "byte_start": 140,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.meta.charset_and_content_type.disallowed",
      "message": "A document must not include both a “meta” element with an “http-equiv” attribute whose value is “content-type”, and a “meta” element with a “charset” attribute.",
      "severity": "Warning",
      "span": {
        "byte_end": 230,
        "byte_start": 140,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “iso-8859-15” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 230,
        "byte_start": 140,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “body”.",
      "severity": "Error",
      "span": {
        "byte_end": 632,
        "byte_start": 625,
        "col": 1,
        "line": 14
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
  "source_name": "html/syntax/parsing/meta-inhead-insertion-mode.html"
}
```
