# html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-no-child-bad-subdomain.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-no-child-bad-subdomain.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Parent is site-keyed, child attempts to origin-key but uses a bad header value, child is a subdomain of the parent</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import {
  insertIframe,
  testSameAgentCluster,
  testGetter
} from "../resources/helpers.mjs";

let frameIndex = 0;
for (const badValue of ["", "?0", "true", "\"?1\"", "1", "?2", "(?1)"]) {
  promise_test(async () => {
    await insertIframe("{{hosts[][www]}}", badValue);
  }, `"${badValue}": frame insertion`);

  // Since the header values are bad they should be site-keyed.
  testSameAgentCluster([self, frameIndex], `"${badValue}"`);
  testGetter(frameIndex, false, `"${badValue}"`);
  ++frameIndex;
}
</script>
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-no-child-bad-subdomain.sub.https.html"
}
```
