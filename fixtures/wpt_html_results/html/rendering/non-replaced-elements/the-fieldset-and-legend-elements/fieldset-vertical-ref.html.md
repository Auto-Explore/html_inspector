# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-vertical-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-vertical-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Reference for fieldset vertical</title>
<link rel=stylesheet href=resources/fieldset-vertical.css>
<p>vertical-lr
<div style="writing-mode: vertical-lr">
 <div class=fieldset><div class="legend top">foo bar</div>normal</div>
 <div class="fieldset rtl"><div class="legend bottom">foo bar</div>dir=rtl</div>
 <div class="fieldset rtl"><div class="legend top">foo bar</div>dir=rtl align=left</div>
 <div class="fieldset rtl"><div class="legend center">foo bar</div>dir=rtl align=center</div>
 <div class="fieldset rtl"><div class="legend bottom">foo bar</div>dir=rtl align=right</div>
 <div class=fieldset><div class="legend top">foo bar</div>align=left</div>
 <div class=fieldset><div class="legend center">foo bar</div>align=center</div>
 <div class=fieldset><div class="legend bottom">foo bar</div>align=right</div>
</div>
<hr>
<p>vertical-rl
<div style="writing-mode: vertical-rl">
 <div class=fieldset><div class="legend top">foo bar</div>normal</div>
 <div class="fieldset rtl"><div class="legend bottom">foo bar</div>dir=rtl</div>
 <div class="fieldset rtl"><div class="legend top">foo bar</div>dir=rtl align=left</div>
 <div class="fieldset rtl"><div class="legend center">foo bar</div>dir=rtl align=center</div>
 <div class="fieldset rtl"><div class="legend bottom">foo bar</div>dir=rtl align=right</div>
 <div class=fieldset><div class="legend top">foo bar</div>align=left</div>
 <div class=fieldset><div class="legend center">foo bar</div>align=center</div>
 <div class=fieldset><div class="legend bottom">foo bar</div>align=right</div></div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-vertical-ref.html"
}
```
