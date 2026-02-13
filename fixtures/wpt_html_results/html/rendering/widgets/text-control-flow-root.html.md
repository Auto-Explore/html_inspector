# html/rendering/widgets/text-control-flow-root.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/text-control-flow-root.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8">
<title>display inside of text control should always be flow-root</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#form-controls">
<link rel=match href="text-control-flow-root-ref.html">

<input id="input1" style="display: inline;">
<br>
<input style="display: inline; transform: scale(0.5);">
<br>
aaa<input style="display: inline;">aaa
<br>
<textarea style="display: inline;"></textarea>
<br>
<textarea style="display: inline; transform: scale(0.5);"></textarea>
<br>
aa<textarea style="display: inline; transform: scale(0.5);">aa</textarea>aa
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/widgets/text-control-flow-root.html"
}
```
