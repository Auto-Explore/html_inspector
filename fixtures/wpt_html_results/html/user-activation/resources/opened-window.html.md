# html/user-activation/resources/opened-window.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/resources/opened-window.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>

<body onload="run()">
  <a id="link" href="?post-nav">link</a>
</body>

<script>
  'use strict';
  const post_nav_page = location.search.substring(1) === "post-nav";

  function sendActivationStateToOpener(msg_type) {
    window.opener.postMessage(JSON.stringify({
        "type": msg_type,
        "isActive": navigator.userActivation.isActive,
        "hasBeenActive": navigator.userActivation.hasBeenActive
    }), "*");
  }

  document.getElementById("link").addEventListener("click", () => {
    assert_false(post_nav_page, "No click in the post-navigation page");
    sendActivationStateToOpener("link-clicked");
  });

  function run() {
    if (!post_nav_page) {
      sendActivationStateToOpener("window-opened");
    } else {
      sendActivationStateToOpener("window-navigated");
    }
  }
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 147,
        "byte_start": 139,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 868,
        "byte_start": 147,
        "col": 9,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 877,
        "byte_start": 868,
        "col": 1,
        "line": 32
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
  "source_name": "html/user-activation/resources/opened-window.html"
}
```
