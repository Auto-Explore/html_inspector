# html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-yes-child-no-same.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-yes-child-no-same.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Parent is origin-keyed, child is site-keyed, child is same-origin to the parent</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import {
  insertIframe,
  testSameAgentCluster,
  testGetter
} from "../resources/helpers.mjs";

promise_setup(async () => {
  await insertIframe("{{hosts[][]}}");
});

// Since they're same-origin, and the parent loaded with origin-keying, the
// child's non-request gets ignored, and both end up origin-keyed.
testSameAgentCluster([self, 0]);

testGetter(self, true, "parent");
testGetter(0, true, "child");
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/1-iframe/parent-yes-child-no-same.sub.https.html"
}
```
