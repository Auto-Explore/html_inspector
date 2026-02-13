# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative-2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>legend position: relative</title>
<link ref=help href="http://crbug.com/1151295">
<link rel=match href=legend-position-relative-2-ref.html>
<style>
#fieldset2 {
  background: lime;
  border: 2px solid lime;
  width: 200px;
  padding: 0;
  margin: 0;
  overflow: hidden;
}
#legend2 {
  position: relative;
  overflow: hidden;
  background: #00ffff;
}
</style>
<p>"Legend" should be shown.</p>
<fieldset id="fieldset2"><legend id="legend2">Legend</legend></fieldset>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.missing_rel_or_itemprop_or_property",
      "message": "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
      "severity": "Warning",
      "span": {
        "byte_end": 104,
        "byte_start": 57,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-position-relative-2.html"
}
```
