# html/browsers/windows/resources/window-name.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/resources/window-name.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>popup helper</title>
<script>

const search = decodeURIComponent(window.location.search.replace("?", ""));
const steps = search.split("|");

async function proceedTest() {
  while (steps.length) {
    const step = steps.shift();

    if (step.startsWith("report=")) {
      const id = step.split("=")[1];
      const stashURL = new URL("window-name-stash.py", location);
      stashURL.searchParams.set('id', id);
      stashURL.searchParams.set('value', window.name);

      await fetch(stashURL, { method: "POST" });
      continue;
    }

    if (step === "close") {
      window.close();
      break;
    }

    if (step === "cross") {
      const url = new URL(window.location);
      url.host = "{{hosts[alt][]}}:{{ports[https][0]}}";
      url.search = "?" + steps.join("|");
      window.location = url.href;
      break;
    }

    if (step === "same") {
      const url = new URL(window.location);
      url.host = "{{host}}:{{ports[https][0]}}";
      url.search = "?" + steps.join("|");
      window.location = url.href;
      break;
    }

    if (step === "sub") {
      const url = new URL(window.location);
      url.host = "{{hosts[][www]}}:{{ports[https][0]}}";
      url.search = "?" + steps.join("|");
      window.location = url.href;
      break;
    }

    if (step === "closeOpener") {
      if (window.opener) {
        window.opener.close();
      }
      continue;
    }

    if (step.startsWith("navOpener=")) {
      if (!window.opener) {
        continue;
      }

      let url = step.split("=")[1];
      window.opener.location.href = url;

      continue;
    }

    if (step === "open") {
      const url = new URL(window.location);
      url.host = "{{host}}:{{ports[https][0]}}";
      url.search = "?" + steps.join("|");
      window.open(url);
      break;
    }

    if (step.startsWith("reportOpener=")) {
      const id = step.split("=")[1];
      const stashURL = new URL("window-name-stash.py", location);
      stashURL.searchParams.set('id', id);
      stashURL.searchParams.set('value', window.opener.name);

      await fetch(stashURL, { method: "POST" });
      continue;
    }

    if (step.startsWith("set=")) {
      window.name = step.split("=")[1];
      continue;
    }

    if (step.startsWith("setDomain=")) {
      document.domain = step.split("=")[1];
      continue;
    }

    throw new Error("Unsupported step!");
  }
}

proceedTest();
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
  "source_name": "html/browsers/windows/resources/window-name.sub.html"
}
```
