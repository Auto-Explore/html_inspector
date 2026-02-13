# html/browsers/origin/cross-origin-objects/frame.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/cross-origin-objects/frame.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
<script>
  if (location.search == "?setdomain") {
    document.domain = document.domain;
  }

  // Override the |frames| and |focus| property to test that such overrides are
  // properly ignored cross-origin.
  window.frames = "override";
  window.focus = "override";

  // Also add a |then| property to test that it doesn't get exposed.
  window.then = "something";
  window.location.then = "something-else";

  // If we get a postMessage, we grab references to everything and set
  // document.domain to trim off our topmost subdomain.
  window.onmessage = function(evt) {
    window.windowReferences = [];
    window.locationReferences = [];
    for (var i = 0; i < parent.length; ++i) {
      windowReferences.push(parent[i]);
      locationReferences.push(parent[i].location);
    }
    try {
      document.domain = document.domain.substring(document.domain.indexOf('.') + 1);
      evt.source.postMessage('PASS', '*');
    } catch (e) {
      evt.source.postMessage('FAIL: cannot trim off document.domain: ' + e, '*');
    }
  }

  function checkWindowReferences() {
    for (var i = 0; i < parent.length; ++i) {
      if (windowReferences[i] != parent[i])
        throw new Error("Window references don't match for " + i + " after document.domain");
      if (locationReferences[i] != parent[i].location)
        throw new Error("Location references don't match for " + i + " after document.domain");
    }
    return true;
  }
</script>
</head>
<body>
  <!-- Two subframes to give us some indexed properties -->
  <iframe></iframe>
  <iframe name=donotleakme></iframe><!-- "donotleakme" is excluded as cross-origin named property due to [[HideFromKeys]] -->
</body>
</html>
```

```json
{
  "messages": [
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
  "source_name": "html/browsers/origin/cross-origin-objects/frame.html"
}
```
