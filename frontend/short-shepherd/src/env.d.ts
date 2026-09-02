declare namespace App {
  interface Locals {
    /** Repo-relative file whose git history the PageTitle override shows.
     *  Set by pages generated from data (they have no markdown source of
     *  their own), so the revision block tracks the data file instead. */
    revisionSource?: string;
  }
}
