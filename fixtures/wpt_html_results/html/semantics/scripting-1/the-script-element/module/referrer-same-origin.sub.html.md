# html/semantics/scripting-1/the-script-element/module/referrer-same-origin.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/referrer-same-origin.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>Referrer with the same-origin policy</title>
<meta name="referrer" content="same-origin">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script type="module">

// "name" parameter is necessary for bypassing the module map.

import { referrer as referrerSame } from "./resources/referrer-checker.py?name=same";

import { referrer as referrerRemote } from "http://{{domains[www1]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/referrer-checker.py?name=remote";

import { referrer as referrerSameSame } from "./resources/import-referrer-checker.sub.js?name=same_same";

import { referrer as referrerSameRemote } from "./resources/import-remote-origin-referrer-checker.sub.js?name=same_remote";

import { referrer as referrerRemoteRemote } from "http://{{domains[www1]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/import-referrer-checker.sub.js?name=remote_remote";

import { referrer as referrerRemoteSame } from "http://{{domains[www1]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/resources/import-same-origin-referrer-checker-from-remote-origin.sub.js?name=remote_same";

const remoteOrigin = "http://{{domains[www1]}}:{{ports[http][0]}}/";

test(t => {
  assert_equals(
      referrerSame, location.href,
      "Referrer should be sent for the same-origin top-level script.");
}, "Importing a same-origin top-level script with the same-origin policy.");

test(t => {
  assert_equals(
      referrerRemote, "",
      "Referrer should not be sent for the remote-origin top-level script.");
}, "Importing a remote-origin top-level script with the same-origin policy.");

test(t => {
  const path =
      new URL("resources/import-referrer-checker.sub.js", location.href);
  assert_equals(
      referrerSameSame, path + `?name=same_same`,
      "Referrer should be sent for the same-origin descendant script.");
}, "Importing a same-origin descendant script from a same-origin top-level " +
   "script with the same-origin policy.");

test(t => {
  assert_equals(
      referrerSameRemote, "",
      "Referrer should not be sent for the remote-origin descendant script.");
}, "Importing a remote-origin descendant script from a same-origin top-level " +
   "script with the same-origin policy.");

test(t => {
  const scriptURL = new URL(
    "html/semantics/scripting-1/the-script-element/module/resources/" +
    "import-referrer-checker.sub.js", remoteOrigin);
  assert_equals(
      referrerRemoteRemote, scriptURL + "?name=remote_remote",
      "Referrer should be sent for the remote-origin descendant script " +
      "when it is imported from a top-level script in the same remote-origin.");
}, "Importing a remote-origin descendant script from a remote-origin " +
   "top-level script with the same-origin policy.");

test(t => {
  assert_equals(
      referrerRemoteSame, "",
      "Referrer should not be sent for the same-origin descendant script " +
      "when it is imported from a top-level remote-origin script.");
}, "Importing a same-origin descendant script from a remote-origin " +
   "top-level script with the same-origin policy.");

</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/module/referrer-same-origin.sub.html"
}
```
