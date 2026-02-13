# html/rendering/non-replaced-elements/the-hr-element-0/size-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-hr-element-0/size-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <style>
        #foo {
            height: 50px;
            box-sizing: border-box;
        }
        #bar {
            border-bottom-width: 0px;
            box-sizing: border-box;
        }
    </style>
</head>
<body>
    <hr id="foo">
    <hr id="bar">
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
  "source_name": "html/rendering/non-replaced-elements/the-hr-element-0/size-ref.html"
}
```
