# html/browsers/history/the-location-interface/non-automated/manual_click_location_replace_during_load-1.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/non-automated/manual_click_location_replace_during_load-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script>
opener.history_length = history.length;
</script>
<a onclick="location.replace('manual_click_location_replace_during_load-2.html'); return false;" href>Click Here</a>
<p>Filler image to keep the page loading:</p>
<img>
<script>
document.images[0].src = "/images/smiley.png?pipe=trickle(20:d1:r2)&random=" + Math.random();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.src_or_srcset.missing",
      "message": "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
      "severity": "Warning",
      "span": {
        "byte_end": 243,
        "byte_start": 238,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 243,
        "byte_start": 238,
        "col": 1,
        "line": 7
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
  "source_name": "html/browsers/history/the-location-interface/non-automated/manual_click_location_replace_during_load-1.html"
}
```
