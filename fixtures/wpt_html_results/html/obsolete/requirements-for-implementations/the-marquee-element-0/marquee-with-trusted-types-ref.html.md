# html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-with-trusted-types-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-with-trusted-types-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>

<!-- See https://github.com/whatwg/html/issues/10249 -->
<!-- This reference matches Gecko's current styles -->

<div style="vertical-align: text-bottom; display: inline-block;">
  <div style="width: 10px; height: 10px; background-color: green;"></div>
</div>
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
  "source_name": "html/obsolete/requirements-for-implementations/the-marquee-element-0/marquee-with-trusted-types-ref.html"
}
```
