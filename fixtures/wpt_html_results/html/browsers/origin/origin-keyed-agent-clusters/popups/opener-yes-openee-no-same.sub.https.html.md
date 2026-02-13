# html/browsers/origin/origin-keyed-agent-clusters/popups/opener-yes-openee-no-same.sub.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/popups/opener-yes-openee-no-same.sub.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Opener is origin-keyed, openee is site-keyed, openee is same-origin to the opener</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>

<script type="module">
import {
  openWindow,
  testOpenedWindowIsInSameAgentCluster,
  testGetter
} from "../resources/helpers.mjs";

let openee;
promise_setup(async () => {
  openee = await openWindow("{{hosts[][]}}");
});

// Since they're same-origin, the openee's non-request is ignored, so both end
// up in the origin-keyed agent cluster.
testOpenedWindowIsInSameAgentCluster(() => openee);

testGetter(self, true, "opener");
testGetter(() => openee, true, "openee");
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/popups/opener-yes-openee-no-same.sub.https.html"
}
```
