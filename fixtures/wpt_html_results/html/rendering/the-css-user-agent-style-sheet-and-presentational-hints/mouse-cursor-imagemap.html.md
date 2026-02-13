# html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/mouse-cursor-imagemap.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/mouse-cursor-imagemap.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<img usemap="#map" src="data:image/gif;base64,R0lGODlhAQABAIAAAOTm7AAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw==" width="1000" height="1000" style="border: 1px solid black;">
<map name="map">
  <area id="clickable-area" shape="rect" coords="0,0,500,500" href="#" role="img">
  <area id="nonclickable-area" shape="rect" coords="500,500,1000,1000" role="img"><!-- No href attribute.-->
</map>

<p>An unclickable (non-link) area should not show the link cursor when hovered.</p>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(function() {
  let clickable = window.getComputedStyle(document.getElementById('clickable-area'));
  let nonclickable = window.getComputedStyle(document.getElementById('nonclickable-area'));
  assert_equals(clickable.getPropertyValue('cursor'), 'pointer');
  assert_not_equals(nonclickable.getPropertyValue('cursor'), 'pointer');
}, 'Only clickable areas should show the link cursor.');
</script>
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
        "byte_end": 184,
        "byte_start": 16,
        "col": 1,
        "line": 2
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
  "source_name": "html/rendering/the-css-user-agent-style-sheet-and-presentational-hints/mouse-cursor-imagemap.html"
}
```
