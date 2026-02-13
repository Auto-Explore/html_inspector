# html/semantics/links/links-created-by-a-and-area-elements/support/target-blank-useractivation.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/links/links-created-by-a-and-area-elements/support/target-blank-useractivation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<a target="_blank" id="a_click_target">Click me</a>
<map id="map">
<area coords="0,0,50,50" target="_blank">
</map>
<img src="/images/blue.png" usemap="#map" style="width: 50px; height: 50px" id="area_click_target">
<script>
let channel = new BroadcastChannel(window.location.search.substring(1));
channel.addEventListener("message", (e) => {
  if (e.data == "close") {
    window.close();
  }
});
channel.postMessage("ready");
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
        "byte_end": 231,
        "byte_start": 132,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “map”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 231,
        "byte_start": 132,
        "col": 1,
        "line": 6
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
  "source_name": "html/semantics/links/links-created-by-a-and-area-elements/support/target-blank-useractivation.html"
}
```
