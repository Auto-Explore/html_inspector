# html/syntax/charset/meta-commented-out.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/charset/meta-commented-out.html",
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
      <script src="/resources/testharness.js"></script>
      <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <script>
      "use strict";
      test(() => {
        assert_not_equals(document.characterSet, "ISO-8859-8");
      }, "document.characterSet must not be determined by commented meta charset");
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
  "source_name": "html/syntax/charset/meta-commented-out.html"
}
```
