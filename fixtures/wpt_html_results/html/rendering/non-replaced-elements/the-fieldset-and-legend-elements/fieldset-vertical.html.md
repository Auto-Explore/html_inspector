# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-vertical.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-vertical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>fieldset vertical</title>
<link rel=stylesheet href=resources/fieldset-vertical.css>
<link rel=match href=fieldset-vertical-ref.html>
<p>vertical-lr
<div style="writing-mode: vertical-lr">
 <fieldset><legend>foo bar</legend>normal</fieldset>
 <fieldset dir=rtl><legend>foo bar</legend>dir=rtl</fieldset>
 <fieldset dir=rtl><legend align=left>foo bar</legend>dir=rtl align=left</fieldset>
 <fieldset dir=rtl><legend align=center>foo bar</legend>dir=rtl align=center</fieldset>
 <fieldset dir=rtl><legend align=right>foo bar</legend>dir=rtl align=right</fieldset>
 <fieldset><legend align=left>foo bar</legend>align=left</fieldset>
 <fieldset><legend align=center>foo bar</legend>align=center</fieldset>
 <fieldset><legend align=right>foo bar</legend>align=right</fieldset>
</div>
<hr>
<p>vertical-rl
<div style="writing-mode: vertical-rl">
 <fieldset><legend>foo bar</legend>normal</fieldset>
 <fieldset dir=rtl><legend>foo bar</legend>dir=rtl</fieldset>
 <fieldset dir=rtl><legend align=left>foo bar</legend>dir=rtl align=left</fieldset>
 <fieldset dir=rtl><legend align=center>foo bar</legend>dir=rtl align=center</fieldset>
 <fieldset dir=rtl><legend align=right>foo bar</legend>dir=rtl align=right</fieldset>
 <fieldset><legend align=left>foo bar</legend>align=left</fieldset>
 <fieldset><legend align=center>foo bar</legend>align=center</fieldset>
 <fieldset><legend align=right>foo bar</legend>align=right</fieldset>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-vertical.html"
}
```
