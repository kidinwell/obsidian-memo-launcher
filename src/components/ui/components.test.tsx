import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { Button } from "./button";
import { Card, CardHeader, CardTitle, CardContent } from "./card";
import { Checkbox } from "./checkbox";
import { Label } from "./label";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "./tabs";

describe("UI Components (Button, Card, Checkbox, Label, Tabs) light theme rendering", () => {
  it("renders Button variants properly", () => {
    render(
      <div>
        <Button variant="default">Primary</Button>
        <Button variant="ghost">Ghost</Button>
        <Button variant="outline">Outline</Button>
      </div>
    );
    expect(screen.getByText("Primary")).toBeDefined();
    expect(screen.getByText("Ghost")).toBeDefined();
    expect(screen.getByText("Outline")).toBeDefined();
  });

  it("renders Card structure", () => {
    render(
      <Card data-testid="card-container">
        <CardHeader>
          <CardTitle>Card Title</CardTitle>
        </CardHeader>
        <CardContent>Content Area</CardContent>
      </Card>
    );
    expect(screen.getByTestId("card-container")).toBeDefined();
    expect(screen.getByText("Card Title")).toBeDefined();
    expect(screen.getByText("Content Area")).toBeDefined();
  });

  it("renders Checkbox and Label", () => {
    render(
      <div className="flex items-center gap-2">
        <Checkbox id="c1" data-testid="test-checkbox" />
        <Label htmlFor="c1">Agree terms</Label>
      </div>
    );
    expect(screen.getByTestId("test-checkbox")).toBeDefined();
    expect(screen.getByText("Agree terms")).toBeDefined();
  });

  it("renders Tabs with tab triggers and contents", () => {
    render(
      <Tabs defaultValue="tab1">
        <TabsList>
          <TabsTrigger value="tab1">Tab 1</TabsTrigger>
          <TabsTrigger value="tab2">Tab 2</TabsTrigger>
        </TabsList>
        <TabsContent value="tab1">Panel 1</TabsContent>
        <TabsContent value="tab2">Panel 2</TabsContent>
      </Tabs>
    );
    expect(screen.getByText("Tab 1")).toBeDefined();
    expect(screen.getByText("Tab 2")).toBeDefined();
    expect(screen.getByText("Panel 1")).toBeDefined();
  });
});
