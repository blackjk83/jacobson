const gitFlashcards = [
    {
        question: "What is Git?",
        answer: "Git is a distributed version control system used to track changes in source code during software development. It allows multiple developers to work on the same project concurrently, manage different versions of the code, and collaborate effectively."
    },
    {
        question: "Explain the difference between 'git pull' and 'git fetch'.",
        answer: "git fetch: Downloads changes from remote repository without merging them into your working branch. Updates local remote-tracking branches.\n\ngit pull: Combination of git fetch and git merge. Downloads changes and automatically merges them into your current branch."
    },
    {
        question: "What is a merge conflict, and how do you resolve it?",
        answer: "A merge conflict occurs when Git cannot automatically merge changes from two branches due to conflicting edits. To resolve:\n1. Open conflicting files\n2. Edit to choose/combine changes\n3. Remove conflict markers\n4. Stage files (git add)\n5. Commit changes"
    },
    {
        question: "What is a .gitignore file?",
        answer: "A .gitignore file specifies intentionally untracked files that Git should ignore. These are typically build artifacts, temporary files, or files containing sensitive information (like API keys) that should not be committed to the repository."
    },
    {
        question: "Explain the difference between git merge and git rebase.",
        answer: "git merge: Creates a merge commit combining changes from two branches. Preserves complete history.\n\ngit rebase: Rewrites commit history by moving your branch's commits to the tip of the target branch. Creates linear history but modifies commit history."
    }
]; 