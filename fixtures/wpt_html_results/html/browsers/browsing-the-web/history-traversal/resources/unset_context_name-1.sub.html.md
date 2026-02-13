# html/browsers/browsing-the-web/history-traversal/resources/unset_context_name-1.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/resources/unset_context_name-1.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<!-- test must be run in a top level browsing context -->
<title>window.name test helper</title>
<script>
const search = window.location.search.replace("?", "");
const steps = search.split("|");

async function proceedTest() {
  while (steps.length) {
    const step = steps.shift();

    if (step.startsWith("report=")) {
      const id = step.split("=")[1];
      const stashURL = new URL("unset_context_name_stash.py", location);
      stashURL.searchParams.set('id', id);
      stashURL.searchParams.set('value', window.name);

      await fetch(stashURL, { method: "POST" });
      continue;
    }

    if (step === "close") {
      window.close();
      break;
    }

    if (step === "navigate") {
      const url = new URL(window.location);
      url.host = "{{hosts[][www]}}:{{ports[http][0]}}";
      url.search = "?" + steps.join("|");
      window.location = url.href;
      break;
    }

    if (step.startsWith("set=")) {
      window.name = step.split("=")[1];
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/resources/unset_context_name-1.sub.html"
}
```
