# html/rendering/the-details-element/details-display-type-001-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-display-type-001-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>Details and summary and display property (subgrid)</title>
<link rel="match" href="details-display-type-001-ref2.html">
<style>

.container {
  display: grid;
  grid: 30px 75px 45px / 80px 125px 90px;
}

.before {
  background: aqua;
  grid-area: 1 / 3;
}

.after {
  background: yellow;
  grid-area: 3 / 1;
}

.details {
  display: grid;
  grid-template: subgrid / subgrid;
  grid-area: 2 / 2 / 4 / 4;
}

.summary {
  background: fuchsia;
  grid-area: 1 / 1;
}

.contents {
  background: silver;
  grid-area: 2 / 2;
}

</style>

<div class="container">
  <div class="before">before</div>
  <div class="details">
    <div class="summary">summary</div>
    <div class="contents">contents</div>
  </div>
  <div class="after">after</div>
</div>
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
  "source_name": "html/rendering/the-details-element/details-display-type-001-ref.html"
}
```
