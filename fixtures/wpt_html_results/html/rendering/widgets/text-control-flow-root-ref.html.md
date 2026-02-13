# html/rendering/widgets/text-control-flow-root-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/text-control-flow-root-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">

<input id="input1">
<br>
<input style="transform: scale(0.5);">
<br>
aaa<input>aaa
<br>
<textarea></textarea>
<br>
<textarea style="transform: scale(0.5);"></textarea>
<br>
aa<textarea style="transform: scale(0.5);">aa</textarea>aa
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
  "source_name": "html/rendering/widgets/text-control-flow-root-ref.html"
}
```
