# html/syntax/charset/meta-charset-no-quotes.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/meta-charset-no-quotes.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
      <!--<meta charset="ISO-8859-8">-->
      <META CHARSET  =  ISO-8859-5  >
      <script src="/resources/testharness.js"></script>
      <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      "use strict";
      test(() => {
        assert_equals(document.characterSet, "ISO-8859-5");
      }, "document.characterSet must be determined by meta charset");
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.meta.charset.mismatch",
      "message": "Internal encoding declaration “ISO-8859-5” disagrees with the actual encoding of the document (“utf-8”).",
      "severity": "Warning",
      "span": {
        "byte_end": 110,
        "byte_start": 79,
        "col": 7,
        "line": 5
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
  "source_name": "html/syntax/charset/meta-charset-no-quotes.html"
}
```
