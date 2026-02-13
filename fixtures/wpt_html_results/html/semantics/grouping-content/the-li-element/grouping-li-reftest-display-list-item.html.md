# html/semantics/grouping-content/the-li-element/grouping-li-reftest-display-list-item.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-display-list-item.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>display: list-item on non-&lt;li> elements</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#ordinal-value">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#list-owner">

<link rel="match" href="grouping-li-reftest-display-list-item-ref.html">

<style>
  .list-item {
    display: list-item;
    list-style-type: decimal;
  }

  .list-item[hidden] {
    display: none;
  }
</style>

<p>This test matches if both lists display similar to the following:</p>

<pre>1. A
2. B
   D
3. E</pre>

<hr>

<ul>
  <span class="list-item">A</span>
  <span class="list-item">B</span>
  <span class="list-item" hidden>C</span>
  <span>D</span>
  <span class="list-item">E</span>
</ul>

<ol>
  <div class="list-item">A</div>
  <div class="list-item">B</div>
  <div class="list-item" hidden>C</div>
  <div>D</div>
  <div class="list-item">E</div>
</ol>
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-display-list-item.html"
}
```
