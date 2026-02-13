# html/rendering/replaced-elements/the-textarea-element/textarea-padding-bstart-moves-content-001-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-textarea-element/textarea-padding-bstart-moves-content-001-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Reference Case</title>

<link rel="stylesheet" href="/fonts/ahem.css">
<style>
textarea {
  font: 10px/1 Ahem;

  block-size: 8em;
  inline-size: 10em;

  padding-block-start: 0;
  padding-block-end: 0;
}
.rtl { direction: rtl; }
.vlr { writing-mode: vertical-lr; }
.vrl { writing-mode: vertical-rl; }
.slr { writing-mode: sideways-lr; }
.srl { writing-mode: sideways-rl; }
</style>

<textarea></textarea>
<textarea class="rtl"></textarea>
<br>
<textarea class="vlr"></textarea>
<textarea class="vrl"></textarea>
<textarea class="slr"></textarea>
<textarea class="srl"></textarea>
<br>
<textarea class="vlr rtl"></textarea>
<textarea class="vrl rtl"></textarea>
<textarea class="slr rtl"></textarea>
<textarea class="srl rtl"></textarea>
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
  "source_name": "html/rendering/replaced-elements/the-textarea-element/textarea-padding-bstart-moves-content-001-ref.html"
}
```
