# html/rendering/the-details-element/summary-display-inline-flex.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/summary-display-inline-flex.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>CSS Test: summary with 'display: inline-flex'</title>
<link rel="author" title="Xing Xu" href="mailto:xing.xu@intel.com">
<link rel="match" href="summary-display-inline-flex-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-details-and-summary-elements">
<meta name="assert" content="Checks that styling a <summary> with 'display: inline-flex' makes it a flex container.">
<style>
.flex-container {
  background: #333;
  border: 0px;
  display: inline-flex;
  margin: 0px;
  padding: 0px;
}

.flex-container.flex-direction-row {
  flex-direction : row;
}

.flex-container.flex-direction-row-reverse {
  flex-direction : row-reverse;
}

.flex-container.flex-direction-column {
  flex-direction : column;
}

.flex-container.flex-direction-column-reverse {
  flex-direction : column-reverse;
}

.flex-container.flex-direction-column-reverse {
  flex-direction : column-reverse;
}

.flex-container.justify-content-center {
  justify-content: center;
}

.flex-container.justify-content-space-around {
  justify-content: space-around;
}

.flex-container.justify-content-space-between {
  justify-content: space-between;
}

.flex-item {
  width:50px;
  height:50px;
  margin:20px;
  background: #eee;
  line-height: 50px;
  text-align: center;
}
</style>

<summary style="display: inline-flex;">
  <div>these fields</div>
  <div>shouldn't be</div>
  <div>stacked vertically</div>
</summary>

<h1>flex-direction: row</h1>
<summary class="flex-container flex-direction-row">
  <div class="flex-item">1</div>
  <div class="flex-item">2</div>
  <div class="flex-item">3</div>
</summary>

<h1>flex-direction: row-reverse</h1>
<summary class="flex-container flex-direction-row-reverse">
  <div class="flex-item">1</div>
  <div class="flex-item">2</div>
  <div class="flex-item">3</div>
</summary>

<h1>flex-direction: column</h1>
<summary class="flex-container flex-direction-column">
  <div class="flex-item">1</div>
  <div class="flex-item">2</div>
  <div class="flex-item">3</div>
</summary>

<h1>flex-direction: column-reverse</h1>
<summary class="flex-container flex-direction-column-reverse">
  <div class="flex-item">1</div>
  <div class="flex-item">2</div>
  <div class="flex-item">3</div>
</summary>

<h1>justify-content: center</h1>
<summary class="flex-container justify-content-center">
  <div class="flex-item">1</div>
  <div class="flex-item">2</div>
  <div class="flex-item">3</div>
</summary>

<h1>justify-content: space-around</h1>
<summary class="flex-container justify-content-space-around">
  <div class="flex-item">1</div>
  <div class="flex-item">2</div>
  <div class="flex-item">3</div>
</summary>

<h1>justify-content: space-between</h1>
<summary class="flex-container justify-content-space-between">
  <div class="flex-item">1</div>
  <div class="flex-item">2</div>
  <div class="flex-item">3</div>
</summary>
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
  "source_name": "html/rendering/the-details-element/summary-display-inline-flex.html"
}
```
