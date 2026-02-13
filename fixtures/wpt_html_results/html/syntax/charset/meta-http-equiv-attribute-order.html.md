# html/syntax/charset/meta-http-equiv-attribute-order.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/meta-http-equiv-attribute-order.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
      <!--<meta http-equiv="Content-Type" content="text/html;charset=ISO-8859-8">-->
      <meta content="text/html;charset=ISO-8859-5" http-equiv="Content-Type"/>
      <script src="/resources/testharness.js"></script>
      <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      "use strict";
      test(() => {
        assert_equals(document.characterSet, "ISO-8859-5");
      }, "document.characterSet must be determined by meta http-equiv");
    </script>
  </body>
</html>
```

```json
{
  "messages": [
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
  "source_name": "html/syntax/charset/meta-http-equiv-attribute-order.html"
}
```
