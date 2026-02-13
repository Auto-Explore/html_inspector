# html/browsers/the-window-object/named-access-on-the-window-object/strict-mode-redefine-readonly-property.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/named-access-on-the-window-object/strict-mode-redefine-readonly-property.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel="help" href="https://crbug.com/1448846">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
test(() => {
  "use strict";
  const pathname = document.location.pathname;
  const form = document.createElement('form');
  form.name = 'location';
  document.body.appendChild(form);
  assert_equals(document.location?.pathname, pathname);
  form.remove();
}, "Adding/removing form with a name referring to a non-configurable property");
</script>
</body>
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
  "source_name": "html/browsers/the-window-object/named-access-on-the-window-object/strict-mode-redefine-readonly-property.html"
}
```
