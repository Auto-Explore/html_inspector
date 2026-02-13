# html/browsers/the-window-object/open-close/resources/message-opener.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/open-close/resources/message-opener.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script src="/common/PrefixedPostMessage.js"></script>
<script>
var prefixedMessage = new PrefixedMessageResource();
var max = 150, attempts = 0;

const urlParams = new URLSearchParams(location.search);
const expected_innerWidth = urlParams.get('expected_innerWidth');
const expected_innerHeight = urlParams.get('expected_innerHeight');
const expected_screenX = urlParams.get('expected_screenX');
const expected_screenY = urlParams.get('expected_screenY');
let should_wait_until_settled = expected_innerWidth === null && expected_innerHeight === null && expected_screenX === null && expected_screenY === null;

function sendCoordinates() {
  // Certain windowing systems position windows asynchronously.
  // As a result, the window may not be positioned yet when the
  // load event fires. To accommodate this, allow waiting up to
  // 15 seconds for positioning to take place.
  if ((!window.screenX && expected_screenX) &&
    (!window.screenY && expected_screenY) && ++attempts < max) {
    setTimeout(sendCoordinates, 100);
    return;
  }
  if (expected_innerWidth && window.innerWidth != expected_innerWidth && ++attempts < max) {
    setTimeout(sendCoordinates, 10);
    return;
  }
  if (expected_innerHeight && window.innerHeight != expected_innerHeight && ++attempts < max) {
    setTimeout(sendCoordinates, 10);
    return;
  }
  if (expected_screenX && window.screenX != expected_screenX && ++attempts < max) {
    setTimeout(sendCoordinates, 10);
    return;
  }
  if (expected_screenY && window.screenY != expected_screenY && ++attempts < max) {
    setTimeout(sendCoordinates, 10);
    return;
  }
  if (should_wait_until_settled) {
    should_wait_until_settled = false;
    setTimeout(sendCoordinates, 300);
    return;
  }
  prefixedMessage.postToOpener({
    left: window.screenX,
    top: window.screenY,
    width: window.innerWidth,
    height: window.innerHeight
  });
}
window.addEventListener('load', sendCoordinates);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 45,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/browsers/the-window-object/open-close/resources/message-opener.html"
}
```
