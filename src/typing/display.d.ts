/**
 * Display mode types for the AuralStudio project.
 *
 * These types describe the possible display states for the UI and rendering modes.
 *
 * Usage:
 *   import type { IDisplay, RDisplay } from './typing/display';
 */

/**
 * Top-level display modes for the application UI.
 * - HOME: Home screen
 * - PROJECTS: Projects list
 * - EDITOR: Editor view
 */
export type IDisplay = "HOME" | "PROJECTS" | "EDITOR";

/**
 * Rendering display modes for panels or floating windows.
 * - FLOAT: Floating window
 * - PANEL: Docked panel
 * - MAX: Maximized view
 */
export type RDisplay = "FLOAT" | "PANEL" | "MAX"; 