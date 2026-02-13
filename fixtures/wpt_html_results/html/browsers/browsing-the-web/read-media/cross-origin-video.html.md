# html/browsers/browsing-the-web/read-media/cross-origin-video.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/read-media/cross-origin-video.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Test cross origin load of media document in parts</title>
<link rel="motivation" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1781759">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/get-host-info.sub.js"></script>
<body></body>
<script>
promise_test(async () => {
  const frame = document.createElement('iframe');
  const dir = location.pathname.replace(/\/[^\/]*$/, '/');
  frame.src =
    // remote origin intermediate document
    get_host_info().HTTP_NOTSAMESITE_ORIGIN + dir
    // iframe-document.sub.html has an iframe with src=childsrc.
    + 'resources/iframe-document.sub.html?childsrc='
    // same origin video document, so that we can play().
    + get_host_info().ORIGIN
    // 'PartialContent' ensures that the entire video resource does not load
    // in one fetch.
    + '/service-workers/service-worker/resources/fetch-access-control.py?'
    + 'VIDEO%26PartialContent';

  let v = document.createElement("video");
  frame.src += "%26mp4";

  document.body.appendChild(frame);
  await new Promise(resolve => frame.onload = resolve);

  const inner = frame.contentWindow.frames[0];
  const video = inner.document.body.childNodes[0];
  video.muted = true;  // to allow playback
  return video.play();
});
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
        "byte_end": 342,
        "byte_start": 334,
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
        "byte_end": 1328,
        "byte_start": 342,
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
        "byte_end": 1337,
        "byte_start": 1328,
        "col": 1,
        "line": 35
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/browsing-the-web/read-media/cross-origin-video.html"
}
```
