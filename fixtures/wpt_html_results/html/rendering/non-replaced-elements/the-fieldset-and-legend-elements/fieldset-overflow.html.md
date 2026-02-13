# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-overflow.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-overflow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<title>fieldset overflow</title>
<link rel=match href=fieldset-overflow-ref.html>
<style>
fieldset, legend {
  border: 1em solid;
  margin: 0;
  padding: 0;
  background: lime
}
#f1 {
  overflow: auto;
  height: 2em;
}
legend {
  height: 1em;
  width: 5em;
}
div {
  background: red;
  height: 2em;
}

#f2 {
  border: none;
  padding: 50px;
  max-height: 50px;
  overflow: scroll;
}

#f3 {
  width: 20em;
  max-height: 250px;
  padding: 7px;
  overflow: auto;
  box-sizing: border-box;
  border-color: transparent;
  background: transparent;
}

#f3 legend {
  height: 40px;
  border: none;
  color: transparent;
  background: transparent;
}
</style>
<p>There should be no red.</p>
<fieldset id="f1">
 <legend></legend>
 <div></div>
 <div id=last></div>
</fieldset>

<!-- crbug.com/1247733 -->
<fieldset id="f2">
  <div style="height:200px; background:blue"></div>
</fieldset>
<script>
 document.getElementById('last').scrollIntoView();
</script>

<!-- crbug.com/1282408 -->
<fieldset id="f3">
  <legend>Legend</legend>
  <p>
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
The quick brown fox jumps over the lazy dog.
  </p>
</fieldset>

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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-overflow.html"
}
```
