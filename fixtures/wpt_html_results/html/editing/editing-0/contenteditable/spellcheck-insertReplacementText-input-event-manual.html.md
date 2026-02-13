# html/editing/editing-0/contenteditable/spellcheck-insertReplacementText-input-event-manual.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/contenteditable/spellcheck-insertReplacementText-input-event-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>
      HTML5 Text Replacement: insertReplacementText Input Event Test
    </title>
    <meta
      content="text/html; charset=UTF-8"
      http-equiv="Content-Type"
    />
    <link
      rel="author"
      title="Microsoft"
      href="http://www.microsoft.com/"
    />
    <link
      rel="help"
      href="https://w3c.github.io/input-events/#overview"
    />
    <meta
      name="assert"
      content="Test insertReplacementText input events and dataTransfer objects"
    />
    <style>
      .test-element {
        border: 2px solid blue;
        width: 200px;
        height: 60px;
        margin: 10px;
        padding: 10px;
      }
      .test-container {
        display: flex;
        flex-wrap: wrap;
        gap: 20px;
        margin: 20px 0;
      }
      .result {
        margin: 5px 0;
        padding: 5px;
        border: 1px solid #ccc;
        font-family: monospace;
        font-size: 12px;
      }
      textarea,
      input {
        width: 180px;
        height: 40px;
      }
      .instructions {
        background-color: #f0f0f0;
        padding: 10px;
        margin: 10px 0;
      }
    </style>
    <script type="text/javascript">
      function logResult(test, pass, details) {
        const div = document.getElementById("results");
        const resultDiv = document.createElement("div");
        resultDiv.className = "result";
        resultDiv.style.color = pass ? "green" : "red";
        resultDiv.innerHTML = `<strong>${test}:</strong> ${
          pass ? "PASS" : "FAIL"
        } - ${details}`;
        div.appendChild(resultDiv);
      }

      function setupReplacementTarget(
        element,
        testName,
        isContentEditable
      ) {
        element.addEventListener("input", function (e) {
          if (e.inputType === "insertReplacementText") {
            if (isContentEditable) {
              // For contenteditable div: expect dataTransfer populated, data as null
              const hasDataTransfer =
                e.dataTransfer !== null && e.dataTransfer !== undefined;
              const dataIsNull = e.data === null;

              if (hasDataTransfer && dataIsNull) {
                logResult(
                  testName,
                  true,
                  `DataTransfer: ${
                    e.dataTransfer.types
                      ? Array.from(e.dataTransfer.types).join(", ")
                      : "none"
                  }, Data: null (correct)`
                );
              } else {
                logResult(
                  testName,
                  false,
                  `Expected dataTransfer populated and data null. Got dataTransfer: ${hasDataTransfer}, data: ${e.data}`
                );
              }
            } else {
              // For input/textarea: expect dataTransfer null, data as text
              const dataTransferIsNull = e.dataTransfer === null;
              const hasData = e.data !== null && e.data !== undefined;

              if (dataTransferIsNull && hasData) {
                logResult(
                  testName,
                  true,
                  `Data: "${e.data}", DataTransfer: null (correct)`
                );
              } else {
                logResult(
                  testName,
                  false,
                  `Expected dataTransfer null and data populated. Got dataTransfer: ${e.dataTransfer}, data: ${e.data}`
                );
              }
            }
          }
        });
      }

      window.onload = function () {
        // Make all divs contenteditable
        document
          .querySelectorAll("div.test-element")
          .forEach((el) => (el.contentEditable = true));

        // Setup input event listeners
        setupReplacementTarget(
          document.getElementById("div-target1"),
          "Spell Check - ContentEditable Div",
          true
        );
        setupReplacementTarget(
          document.getElementById("textarea-target1"),
          "Spell Check - Textarea",
          false
        );
        setupReplacementTarget(
          document.getElementById("input-target1"),
          "Spell Check - Input",
          false
        );
      };
    </script>
  </head>
  <body>
    <h1>insertReplacementText Input Event Test</h1>
    <p>
      Test insertReplacementText input events and their dataTransfer
      objects during text replacement operations.
    </p>

    <div class="instructions">
      <h3>Test Instructions:</h3>
      <ol>
        <li>
          <strong>Spell Check:</strong> Type a misspelled word,
          right-click and select a correction
        </li>
      </ol>
    </div>

    <div class="test-container">
      <!-- Spell Check Tests -->
      <div>
        <h4>Spell Check - ContentEditable Div</h4>
        <div id="div-target1" class="test-element" spellcheck="true">
          Type "teh" or "recieve" and right-click for corrections
        </div>
      </div>

      <div>
        <h4>Spell Check - Textarea</h4>
        <textarea
          id="textarea-target1"
          class="test-element"
          spellcheck="true"
          placeholder="Type 'teh' or 'recieve' and right-click for corrections"
        ></textarea>
      </div>

      <div>
        <h4>Spell Check - Input</h4>
        <input
          id="input-target1"
          class="test-element"
          spellcheck="true"
          placeholder="Type 'teh' or 'recieve' and right-click for corrections"
        />
      </div>
    </div>

    <div class="instructions">
      <h4>How to test:</h4>
      <ul>
        <li>
          <strong>Spell Check:</strong> Type misspelled words like "teh"
          or "recieve", right-click and select correction
        </li>
      </ul>
    </div>

    <h3>insertReplacementText Results:</h3>
    <div id="results">
      Perform text replacement operations above to see PASS/FAIL
      results...
    </div>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 1211,
        "byte_start": 1180,
        "col": 5,
        "line": 56
      }
    },
    {
      "category": "Html",
      "code": "html.heading.skip_level",
      "message": "The heading “h3” (with computed level 3) follows the heading “h1” (with computed level 1), skipping 1 heading level.",
      "severity": "Warning",
      "span": {
        "byte_end": 4485,
        "byte_start": 4481,
        "col": 7,
        "line": 154
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
  "source_name": "html/editing/editing-0/contenteditable/spellcheck-insertReplacementText-input-event-manual.html"
}
```
