# Refactor Skills: Rust Concept-Demo Code

## Goal
Refactor “messy learning/demo code” into small, concept-focused functions that are correct, skimmable, and idiomatic for Rust learners.

## Constraints Checklist (apply to each refactor)
- **Concept-first structure**
  - Extract each *single concept* into its own function.
  - `main` should be a short, ordered list of concept function calls.

- **Rust correctness**
  - Verify every statement in comments matches Rust semantics.
  - Prefer Rust terms:
    - “final/last expression is returned” (vs “implicit return”).
    - “initializer” / “const expression” for `const` rules.

- **Beginner relevance**
  - Explain only what a learner needs to understand the snippet.
  - Teach the “why” at the point of use (e.g., statements vs expressions).

- **Concise + consistent comments**
  - One idea per comment line.
  - Avoid paragraphs; prefer 1–3 short lines above a block.
  - Use consistent vocabulary across the file (don’t rename the same concept).

- **Skimmability**
  - Function names should communicate the concept without reading the body.
  - Keep demo functions short; remove incidental complexity.
  - Prefer whitespace and small blocks over dense inline explanations.

- **Concept-relevant naming**
  - Avoid vague names like `something`.
  - Name functions after the concept being demonstrated (e.g., `expression_return_example`).

## Quick “LLM action” prompts
- **Extract concepts**: “Identify each distinct concept in `main`, create one function per concept, then call them in `main` in the same order.”
- **Rewrite comments**: “Rewrite comments to be Rust-correct, beginner-relevant, concise, consistent, and skimmable.”
- **Rename for concept**: “Rename functions/vars so the name reflects the concept being demonstrated; update all call sites.”

## Validation
- **Compilation**: Ensure the refactor still compiles.
- **Behavior**: Preserve printed output and example behavior unless explicitly requested otherwise.
