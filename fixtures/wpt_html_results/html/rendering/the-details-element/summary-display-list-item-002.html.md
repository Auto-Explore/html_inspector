# html/rendering/the-details-element/summary-display-list-item-002.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/summary-display-list-item-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>CSS Test: summary with 'display: list-item' and flex children</title>
<link rel="author" title="Cameron McCormack" href="mailto:heycam@apple.com">
<link rel="author" title="Sergio Villar Senin" href="mailto:svillar@igalia.com">
<link rel="match" href="summary-display-list-item-002-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-details-and-summary-elements">
<style>
summary {
    display: list-item;
    list-style-type: none;
}
/* WebKit does not support list-style-type:none yet */
summary::-webkit-details-marker { display: none; }
</style>
<details style="width: 100px;">
  <summary>
    <div style="display: flex; justify-content: space-between;">
      <div>AAA</div>
      <div>BBB</div>
    </div>
  </summary>
</details>
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
  "source_name": "html/rendering/the-details-element/summary-display-list-item-002.html"
}
```
