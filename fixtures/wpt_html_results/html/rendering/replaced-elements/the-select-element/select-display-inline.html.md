# html/rendering/replaced-elements/the-select-element/select-display-inline.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-display-inline.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>&lt;select&gt; with display:inline</title>
<link rel="author" title="Oriol Brufau" href="obrufau:obrufau@igalia.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-select-element-2">
<link rel="help" href="https://github.com/servo/servo/issues/39927">
<link rel="match" href="select-display-inline-ref.html">
<meta name="assert" content="
  A <select> is a widget, so it's atomic when it has display:inline.
  In particular, this implies that it's transformable.
">

<select style="display: inline; transform: translateX(100px)">
  <option>option</option>
</select>
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
  "source_name": "html/rendering/replaced-elements/the-select-element/select-display-inline.html"
}
```
