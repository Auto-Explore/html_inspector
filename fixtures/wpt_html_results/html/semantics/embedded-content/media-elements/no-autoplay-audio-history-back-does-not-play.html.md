# html/semantics/embedded-content/media-elements/no-autoplay-audio-history-back-does-not-play.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/media-elements/no-autoplay-audio-history-back-does-not-play.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>No autoplay audio does not start after history back</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
  <script>
    "use strict";

    promise_test(async t => {
      const base = location.pathname.replace(/[^/]*$/, "");

      const iframe = document.createElement("iframe");
      document.body.appendChild(iframe);
      t.add_cleanup(() => iframe.remove());

      function waitForMessage(type, timeoutMs) {
        let timeoutId;
        let onMessage;

        const timeoutPromise = new Promise((_, reject) => {
          timeoutId = t.step_timeout(() => {
            window.removeEventListener("message", onMessage);
            reject(new Error(`Timed out waiting for ${type}`));
          }, timeoutMs);
        });

        const messagePromise = new Promise((resolve) => {
          onMessage = (event) => {
            if (event.source !== iframe.contentWindow) return;
            if (!event.data || event.data.type !== type) return;

            clearTimeout(timeoutId);
            window.removeEventListener("message", onMessage);
            resolve(event.data);
          };

          window.addEventListener("message", onMessage);
        });

        return Promise.race([timeoutPromise, messagePromise]);
      }

      iframe.src = `${base}resources/no-autoplay-audio-history-back-does-not-play-a.html`;
      await waitForMessage("ready_a", 3000);


      iframe.src = `${base}resources/no-autoplay-audio-history-back-does-not-play-b.html`;
      await waitForMessage("loaded_b", 3000);


      iframe.contentWindow.history.back();


      const result = await waitForMessage("result", 3000);

      assert_true(result.pass, JSON.stringify(result.details));
    }, "No autoplay audio remains paused after iframe history back");

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
  "source_name": "html/semantics/embedded-content/media-elements/no-autoplay-audio-history-back-does-not-play.html"
}
```
