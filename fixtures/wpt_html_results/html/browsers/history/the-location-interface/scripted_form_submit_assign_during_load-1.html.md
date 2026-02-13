# html/browsers/history/the-location-interface/scripted_form_submit_assign_during_load-1.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/scripted_form_submit_assign_during_load-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script>
opener.test_initial_history_length(history.length);
</script>
<form onsubmit="location = 'scripted_form_submit_assign_during_load-2.html'; return false;">
<input type=submit value="Click Me">
</form>
<script>
document.forms[0].elements[0].click()
</script>
<p>Filler image to keep the page loading:</p>
<img src="/images/smiley.png?pipe=trickle(20:d1:r2)">
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 381,
        "byte_start": 328,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/history/the-location-interface/scripted_form_submit_assign_during_load-1.html"
}
```
