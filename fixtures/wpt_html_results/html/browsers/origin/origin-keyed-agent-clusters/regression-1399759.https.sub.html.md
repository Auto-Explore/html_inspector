# html/browsers/origin/origin-keyed-agent-clusters/regression-1399759.https.sub.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/regression-1399759.https.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta name="variant" content="?pipe=header(Origin-Agent-Cluster,%3F0)">
<meta name="variant" content="?pipe=header(Origin-Agent-Cluster,%3F1)">
<title>Origin-Isolation after navigating about:blank.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
</head>
<body>
</body>
<script>
// Regression test for crbug.com/1399759. This is mainly based on
// external/wpt/html/infrastructure/urls/base-url/document-base-url-initiated-grand-parent.https.window.js
// but restricts itself to the exact error condition.
//
// This test is run in two variants which differ in the Origin-Agent-Cluster
// http header values, ?0 and ?1. The test should pass in either case, but the
// regression we're testing for involves inconsistent clustering decisions,
// which requires clustering to be enabled in the first place.
promise_test(async test => {
  // Create a cross-origin iframe. Use the executor.html, so we can ask it
  // to execute scripts for us.
  const child_token = token();
  const iframe = document.createElement("iframe");
  iframe.src = get_host_info().HTTPS_REMOTE_ORIGIN +
    `/common/dispatcher/executor.html?uuid=${child_token}`;
  document.body.appendChild(iframe);

  // The child creates a grand child in an iframe.
  const reply_token = token();
  send(child_token, `
    const iframe = document.createElement("iframe");
    iframe.src = "/common/blank.html";
    iframe.onload = () => {
      send("${reply_token}", "grand child loaded");
    };
    document.body.appendChild(iframe);
  `);
  assert_equals(await receive(reply_token), "grand child loaded");
  const grandchild = iframe.contentWindow[0];

  // Navigate the grand-child toward about:blank.
  grandchild.location = "about:blank";
  assert_equals(await receive(reply_token), "grand child loaded");

  // This document and grandchild are same-origin, because about:blank
  // inherits its origin from the initiator of the navigation, which is us.
  // This access should not throw.
  grandchild.document;
}, "Check the baseURL of an about:blank document cross-origin with its parent");

promise_test(async test => {
  // This tests the same setup as above, but with about:srcdoc. Since one
  // cannot just navigate to about:srcdoc, we'll have to include an extra
  // step: Create an iframe with srcdoc attribute; navigate away; then
  // navigate to about:srcdoc.
  // srcdoc does not inherit the origin from the initiator - unlike
  // about:blank - and so in this case the grandchild.document access should
  // throw.

  // Create a cross-origin iframe. Use the executor.html, so we can ask it
  // to execute scripts for us.
  const child_token = token();
  const iframe = document.createElement("iframe");
  iframe.src = get_host_info().HTTPS_REMOTE_ORIGIN +
    `/common/dispatcher/executor.html?uuid=${child_token}`;
  document.body.appendChild(iframe);

  // The child creates a grand child in an iframe, using the srcdoc attribute.
  const reply_token = token();
  send(child_token, `
    const iframe = document.createElement("iframe");
    iframe.onload = () => {
      send("${reply_token}", "grand child loaded");
    };
    iframe.srcdoc = "nothing interesting";
    document.body.appendChild(iframe);
  `);
  assert_equals(await receive(reply_token), "grand child loaded");
  const grandchild = iframe.contentWindow[0];

  // Navigate the grand child toward a regular URL.
  grandchild.location = get_host_info().HTTPS_REMOTE_ORIGIN + "/common/blank.html";
  assert_equals(await receive(reply_token), "grand child loaded");

  // Navigate the grand-child back, to about:srcdoc.
  grandchild.location = "about:srcdoc";
  assert_equals(await receive(reply_token), "grand child loaded");

  // This document and grandchild are cross-origin. about:srcdoc does not
  // inherits its origin from the initiator of the navigation. This access
  // should throw:
  assert_throws_dom("SecurityError", () => { grandchild.document; });
}, "Check that about:srcdoc navigation does not follow about:blank rules.");
</script>
</html>
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
        "byte_end": 524,
        "byte_start": 516,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 4236,
        "byte_start": 524,
        "col": 9,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 4245,
        "byte_start": 4236,
        "col": 1,
        "line": 99
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/regression-1399759.https.sub.html"
}
```
