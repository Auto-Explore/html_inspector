# html/webappapis/dynamic-markup-insertion/closing-the-input-stream/document-close-with-pending-script.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/closing-the-input-stream/document-close-with-pending-script.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>document.close called while a script is pending</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<body>
  <script>
    window.t = async_test();
    // We want start a document load, create an non-blocking script load inside
    // it, have the parser complete, then call document.open()/document.close()
    // after the parser is done but before the non-blocking script load
    // completes.  After we do that, the document should reach the 'complete'
    // ready state and the iframe's load event should fire.
    var loadFired = false;
    var i;

    var finish = t.step_func_done(() => {
      assert_equals(loadFired, true, "Should have fired a load event");
      assert_equals(i.contentDocument.readyState, "complete",
                    "Should be fully loaded");
    });

    var checkForLoad = t.step_func(() => {
      if (loadFired) {
        finish();
      } else {
        i.addEventListener("load", finish);
      }
    });

    window.parserDone = t.step_func(() => {
      var doc = i.contentDocument;
      i.onload = () => { loadFired = true; }
      doc.open();
      doc.close();
      // It's not very clearly specced whether the document is
      // supposed to be fully loaded at this point or not, so allow
      // that to be the case, or to happen soonish.
      assert_true(doc.readyState == "interactive" ||
                  doc.readyState == "complete", "Should be almost loaded");
      if (doc.readyState == "complete") {
        checkForLoad();
      } else {
        doc.addEventListener("readystatechange", checkForLoad);
      }
    });

    t.step(() => {
        i = document.createElement("iframe");
        i.srcdoc = `
          <script>
            parent.t.step(() => {
              var s = document.createElement("script");
              s.src = "/common/slow.py";
              document.documentElement.appendChild(s);
              // Call into the parent async, so we finish our "end of parse"
              // work before it runs.
              document.addEventListener(
                "DOMContentLoaded",
                () => parent.t.step_timeout(parent.parserDone, 0));
            });
          <\/script>
        `;
        document.body.appendChild(i);
    });
  </script>
</body>
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
  "source_name": "html/webappapis/dynamic-markup-insertion/closing-the-input-stream/document-close-with-pending-script.html"
}
```
