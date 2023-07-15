import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PdfDetailViewComponent } from './pdf-detail-view.component';

describe('PdfDetailViewComponent', () => {
  let component: PdfDetailViewComponent;
  let fixture: ComponentFixture<PdfDetailViewComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ PdfDetailViewComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PdfDetailViewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
