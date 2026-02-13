# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/video-inline-size-containment-no-crash.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/video-inline-size-containment-no-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<link author="Sammy Gill" href="mailto:sammy.gill@apple.com">
<meta name="assert" content="Applying inline-size containment to the video should not result in a debug assert being triggered">
<style>
video {
  aspect-ratio: 1;
  container-type: inline-size;
  inset: 0 auto;
  min-width: min-content;
  position: fixed;
}
</style>
</head>
<body>
<video></video>
</body>
</html>
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
        "byte_end": 91,
        "byte_start": 30,
        "col": 1,
        "line": 4
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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/video-inline-size-containment-no-crash.html"
}
```
